//use crate::models::types::Pool;

//pub async fn get_all_photos_full(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
//
//}

//pub async fn get_all_photos_full(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
//    // execute sync code in threadpool
//    let res = web::block(move || {
//        let mut conn = db.get().unwrap();
//
//        let query = "SELECT * FROM photos_all";
//
//        for row in &conn.query(query, &[]).unwrap()
//    })
//}
