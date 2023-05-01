

impl FromRequest for User {
    type Error = MyError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let token = req.headers().get("Authorization");
            let db = req.app_data::<Data<AppState>>();
            if db .is_none() {
                return Err(MyError::InternalError);
            }

            return match token {
                Some(data) => {
                    let db = db.unwrap().clone();
                    let authToken = token.unwrap().to_str().unwrap().clone();

                    let x=&authToken[7..];
                    let res = sqlx::query_as!(Auth,"select token,user_id from auth where token =$1",x)
                        .fetch_one(&db.pool).await;
                    let auth = res;
                    match auth{
                        Ok(a)=>{
                            let res = sqlx::query_as!(User,"select u.id,u.username,u.password from users u inner join Auth A on u.id = A.user_id where id=$1",a.user_id)
                                .fetch_one(&db.pool).await;
                            Ok(res.unwrap())
                        },
                        _=>Err(MyError::InternalError)
                    }
                   }

                _ => Err(MyError::UnAuthorized)
            }
        })
}}