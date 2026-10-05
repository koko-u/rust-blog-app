# realm

- blog-app

# client

- blog-app
    - blog-app-spa
    - blog-app-api

| client       | Role on OAuth   | client authentication | Standard Flow | PKCE | Valid redirect URIs     | Web Origins           |
|--------------|-----------------|-----------------------|---------------|------|-------------------------|-----------------------|
| blog-app-spa | OAuth Client    | OFF                   | ON            | ON   | http://localhost:4200/* | http://localhost:4200 |
| blog-app-api | Resource Server | OFF                   | OFF           | -    | -                       | -                     |

## blog-app-spa

SPA はブラウザで動作するので、Client Secret を安全に保存できない
このため、Public Client として、Authentication Code Flow + PKCE を使用する

この Client は OAuth / OIDC の Client と同じ意味である

## blog-app-api

OAuth のクライアントとしての役割はないので、原則すべての設定をOFFしてクライアントを作成する。
クライアント別のロールを用意することで、post:read や post:write といった権限を設定できる。

# Realm Roles

- admin
- user

# audience の設定

これは Protocol Mapper がアクセストークンを生成するときに何を "aud" に入れるかを決定している

Clients > blog-app-api > Client Scopes > blog-app-api-decicated

(なんたら-dedicated が keycloak が自動で作成しているデフォルトのクライアントの Client Scope )

そこから Mappers を新規に作成する -> Configure New Mapper
そこにようやく Audience の設定が表われるので、選択する

| name              | included client audience | add to access token |
|-------------------|--------------------------|---------------------|
| blog-app-audience | blog-app-api (選択式)    | ON                  |



