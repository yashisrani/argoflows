use argoflows::api::info;
use argoflows::config::Config;
use argoflows::types::info::CollectEventRequest;

fn main() {
    let token = std::env::var("ARGO_TOKEN").expect("the ARGO_TOKEN env variable must be set");

    let cfg = Config::builder()
        .bearer_token(&token)
        .danger_accept_invalid_certs(true)
        .build();
    let cfg = cfg.expect("failed to create client config");

    let payload = r#"{
        "type": "custom-event",
        "source": "ci-pipeline",
        "data": {
            "workflow": "event-driven-workflow",
            "trigger": "build-success",
            "message": "Build completed successfully!"
        }
    }"#;
    match info::collect_event(&cfg, CollectEventRequest::new(payload)) {
        Ok(r) => println!("{:?}\n", r),
        Err(e) => eprintln!("failed to get info: {:?}", e),
    }

    match info::get_info(&cfg) {
        Ok(r) => println!("{:?}\n", r),
        Err(e) => eprintln!("failed to get info: {:?}", e),
    }

    match info::get_user_info(&cfg) {
        Ok(r) => println!("{:?}\n", r),
        Err(e) => eprintln!("failed to get user info: {:?}", e),
    }

    match info::get_version(&cfg) {
        Ok(v) => println!("{:?}\n", v),
        Err(e) => eprintln!("failed to get version: {:?}", e),
    }
}
