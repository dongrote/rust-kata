use std::{path::Path, sync::{Mutex, Arc}, env};

use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};
use timetrack::types::timetrackerstores::TimeTrackerFileStore;

struct AppState {
    pub time_tracker_store: Arc<Mutex<TimeTrackerFileStore>>,
}

#[get("/ui")]
async fn ui() -> impl Responder {
    let body = String::from("
    <html>
    <head>
    <style>
    body {background-color: black;}
    h1 {color: white; font-size: 10em; font-family: system-ui; font-style: italic;}
    </style>
    </head>
    <body>
        <h1></h1>
        <script>
            const updateTime = () => {
                const h = document.getElementsByTagName('h1')[0];
                fetch('/elapsed')
                    .then(res => res.body.getReader().read())
                    .then(data => {
                        let seconds = parseInt(new TextDecoder().decode(data.value));
                        const hours = Math.floor(seconds / 3600);
                        seconds -= hours * 3600;
                        const minutes = Math.floor(seconds / 60);
                        const minutesString = `${minutes}`.padStart(2, '0');
                        seconds -= minutes * 60;
                        const secondsString = `${seconds}`.padStart(2, '0');
                        h.textContent = `${hours}:${minutesString}:${secondsString}`;
                        setTimeout(() => updateTime(), 1000);
                    })
                    .catch(console.error);
            };
            updateTime();
        </script>
    </body>
</html>
");
    HttpResponse::Ok().insert_header(ContentType::html()).body(body)
}

#[get("/elapsed")]
async fn elapsed(data: web::Data<AppState>) -> String {
    let store = data.time_tracker_store.lock().unwrap();
    format!("{}", store.duration().to_string())
}

#[put("/resume")]
async fn resume(data: web::Data<AppState>) -> impl Responder {
    let mut store = data.time_tracker_store.lock().unwrap();
    match store.resume() {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[put("/stop")]
async fn stop(data: web::Data<AppState>) -> impl Responder {
    let mut store = data.time_tracker_store.lock().unwrap();
    match store.stop() {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let timer_path = match args.len() > 1 {
        true => Path::new(&args[1]),
        false => Path::new("foo.txt"),
    };
    match TimeTrackerFileStore::from_file(timer_path) {
        Ok(time_tracker_store) => {
            let rc_time_tracker_store = Arc::new(Mutex::new(time_tracker_store));
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(AppState {
                        time_tracker_store: Arc::clone(&rc_time_tracker_store),
                    }))
                    .service(elapsed)
                    .service(resume)
                    .service(stop)
                    .service(ui)
            })
            .bind(("0.0.0.0", 8080))?
            .run()
            .await
        },
        Err(err) => Err(err),
    }
}
