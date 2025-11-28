pub(crate) mod repository {

    use super::super::models::model::dto::GENERIC;
    pub struct Repository {}

    impl Repository {
        pub fn new() -> Self {
            Self {}
        }

        pub fn get_all(&self) -> Result<Vec<GENERIC>, ()> {
            let list: Vec<GENERIC> = vec![GENERIC { id: 1 }, GENERIC { id: 2 }, GENERIC { id: 3 }];
            Ok(list)
        }

        pub fn get_by_id(&self, _id: i32) -> Result<Vec<GENERIC>, ()> {
            let list: Vec<GENERIC> = vec![GENERIC { id: 1 }, GENERIC { id: 2 }, GENERIC { id: 3 }];
            let mut result: Vec<GENERIC> = Vec::new();
            for item in list {
                if item.id == _id {
                    result.push(item);
                }
            }
            Ok(result)
        }

        pub fn create(&self, _dto: GENERIC) -> Result<GENERIC, ()> {
            Ok(_dto)
        }

        pub fn update(&self, _id: i32, _dto: GENERIC) -> Result<(i32, GENERIC), ()> {
            Ok((_id, _dto))
        }

        pub fn delete(&self, _id: i32) -> Result<bool, ()> {
            let list: Vec<GENERIC> = vec![GENERIC { id: 1 }, GENERIC { id: 2 }, GENERIC { id: 3 }];

            for item in list {
                if item.id == _id {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    }
}
