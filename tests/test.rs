use std::{
    env::{
        self
    },
    sync::{
        Arc
    }
};
use tracing::{
    debug,
    // info
};
use reqwest::{
    Client
};
use tracing_subscriber::{
    prelude::{
        *
    }
};
use pocket_api_client::{
    PocketApiConfig,
    PocketApiTokenReceiver,
    PocketApiClient
};

fn initialize_logs() {
    // Логи в stdout
    let stdoud_sub = tracing_subscriber::fmt::layer()
        .pretty()
        .with_test_writer();

    // Суммарный обработчик
    let full_subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::default()
                .add_directive(tracing::Level::DEBUG.into())
                .and_then(stdoud_sub));

    // Установка по-умолчанию
    tracing::subscriber::set_global_default(full_subscriber).unwrap();
}

async fn receive_token(config: Arc<PocketApiConfig>) -> String{
// Получатель токена
    let token_receiver = PocketApiTokenReceiver::new(config.clone());

    // Инфа по аутентфикации и авторизации пользователя
    let auth_info = token_receiver
        .optain_user_auth_info("http://127.0.0.1:9999/callback".to_string())
        .await
        .expect("Auth info receive failed");
    debug!("Auth info: {}", auth_info);

    // Открываем браузер для подтверждения
    webbrowser::open(auth_info.auth_url.as_str())
        .expect("Browser open failed");

    println!("Confirm autorization in browser and continue");

    // Запускаем сервер и ждем коллбека
    let sender = std::sync::Arc::new(tokio::sync::Notify::new());
    let receiver = sender.clone();
    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel::<()>();
    let j = tokio::spawn(async move {
        use warp::Filter;
        let routes = warp::path!("callback")
            .map(move || {
                sender.notify_waiters();
                ""
            });
        let (_addr, server) = warp::serve(routes)
            .bind_with_graceful_shutdown(([127, 0, 0, 1], 9999), async {
                stop_rx.await.ok();
            });
        server.await
    });
    receiver.notified().await;
    stop_tx.send(()).unwrap();
    j.await.unwrap();

    // Непосредственно получение токена
    let token = token_receiver
        .receive_token(auth_info.code)
        .await
        .expect("Token receive failed");
    debug!("Received token: {}", token);

    token
}

#[tokio::test]
async fn library_integration_test(){
    initialize_logs();

    dotenv::from_filename(".test.env").expect("Environment file reading failed");

    // Переменные окружения
    let pocket_consumer_id = env::var("POCKET_CONSUMER_ID").expect("Missing env variable");

    // Общий конфиг
    let config = Arc::new(PocketApiConfig::new_default(Client::new(), pocket_consumer_id));

    // Токен пользователя
    let user_token = receive_token(config.clone()).await;

    // Клиент
    let api_client = PocketApiClient::new(config, user_token);

    // Добавляем итем
    let item = api_client
        .add("https://google.com".to_string(), None)
        .await
        .expect("Insert failed");
    debug!("New item: {:#?}", item);

    // Список итемов
    let all_items = api_client
        .get_all()
        .await
        .expect("Get all failed");
    debug!("All items: {:#?}", all_items);
}