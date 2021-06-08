use martlet_common::service::Service;
use martlet_proxy::service::mysql::MySQLService;

pub fn new_service() -> Box<&'static dyn Service> {
    Box::new(&MySQLService {} as &Service)
}