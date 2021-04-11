use std::{
    env::{
        self
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
    PocketApiTokenReceiver
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

#[tokio::test]
async fn library_integration_test(){
    initialize_logs();

    dotenv::from_filename(".test.env").expect("Environment file reading failed");

    // Переменные окружения
    let pocket_consumer_id = env::var("POCKET_CONSUMER_ID").expect("Missing env variable");

    // Создаем HTTP клиента, можно спокойно клонировать, внутри Arc
    let http_client = Client::new();

    let client = PocketApiTokenReceiver::new(http_client, pocket_consumer_id);

    let auth_info = client
        .optain_user_auth_info("http://127.0.0.1:9999/callback".to_string())
        .await
        .expect("Auth info receive failed");
    debug!("Auth info: {}", auth_info);

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

    let token = client
        .receive_token(auth_info.code)
        .await
        .expect("Token receive failed");
    debug!("Received token: {}", token);
}