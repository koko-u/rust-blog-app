## routings

|            | method | path                               | description                    |
|------------|--------|------------------------------------|--------------------------------|
| categories | GET    | /api/categories                    | Get All Categories             |
|            |        | /api/categories/{id}               | Get Single Category            |
|            | POST   | /api/categories                    | Create Category                |
|            | PUT    | /api/categories/{id}               | Update Category                |
|            | DELETE | /api/categories/{id}               | Delete the Category            |
| tags       | GET    | /api/tags                          | Get All Tags                   |
|            |        | /api/tags/{id}                     | Get Single Tag                 |
|            | POST   | /api/tags                          | Create Tag                     |
|            | PUT    | /api/tags/{id}                     | Update Tag                     |
|            | DELETE | /api/tags/{id}                     | Delete the Tag                 |
| posts      | GET    | /api/posts                         | Get All Posts                  |
|            |        | /api/posts/{id}                    | Get Single Post                |
|            | POST   | /api/posts                         | Create Post                    |
|            | PUT    | /api/posts/{id}                    | Update Post                    |
|            | DELETE | /api/posts/{id}                    | Delete the Post                |
| comments   | GET    | /api/posts/{post_id}/comments      | Get All Comments of the post   |
|            |        | /api/posts/{post_id}/comments/{id} | Get Single Comment of the post |
|            | POST   | /api/posts/{post_id}/comments      | Create Comment on the post     |
|            | PUT    | /api/posts/{post_id}/comments/{id} | Update Comment                 |
|            | DELETE | /api/posts/{post_id}/comments/{id} | Delete the Comment             |