use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use db::models::news_feed_url::NewsFeedUrlSlug;

use db::sql::news_feed_url::find_news_feed_url_slugs_within_date_range;

use sqlx::postgres::PgPool;

pub async fn sitemap_news_feed_url_slugs_query<'a>(db_pool: &PgPool, _month: i32, _year: i32) -> FieldResult<Vec<NewsFeedUrlSlug>> {
    // convert ints to datetime
    // Add 1 month to date time
    // Query for url_slugs between period
    // Map NewsFeedUrlSlug to string

    match find_news_feed_url_slugs_within_date_range(db_pool, 1, 2).await {
        Ok(news_feed_urls) => Ok(news_feed_urls),
        Err(_) => Err(GqlError::NotFound.extend()),
    }
}

#[cfg(test)]
mod tests {

    use crate::graphql::test_util::create_fake_schema;
    use async_graphql::value;
    use db::{
        init_env, init_test_db_pool,
        util::{
            convert::now_utc_timestamp,
            test::test_util::{
                create_fake_news_feed_url, create_fake_news_tweet_url,
                create_fake_news_twitter_user,
            },
        },
    };

    #[tokio::test]
    async fn get_news_feed_urls_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        let created_at_timestamp = now_utc_timestamp();
        create_fake_news_tweet_url(&db_pool, created_at_timestamp).await;
        create_fake_news_feed_url(&db_pool, created_at_timestamp).await;
        create_fake_news_twitter_user(&db_pool).await;

        let schema = create_fake_schema(db_pool);

        let resp = schema
            .execute(
                r#"
                query {
                    newsFeedUrls {
                        urlSlug
                        urlId
                        urlScore
                        numReferences
                        firstReferencedByUsername
                        createdAt
                        title
                        description
                        expandedUrlParsed
                        expandedUrlHost
                        previewImageThumbnailUrl
                        previewImageUrl
                        displayUrl
                      }
                }
                "#,
            )
            .await;
        assert_eq!(
            resp.data,
            value!({
                "newsFeedUrls": [
                    {
                        "urlSlug": String::from("example-title"),
                        "urlId": 1,
                        "urlScore": 90,
                        "numReferences": 2,
                        "firstReferencedByUsername": String::from("username"),
                        "createdAt": now_utc_timestamp(),
                        "title": String::from("example title"),
                        "description": String::from("description"),
                        "expandedUrlParsed": String::from("expanded_url_parsed"),
                        "expandedUrlHost": String::from("expanded_url_host"),
                        "previewImageThumbnailUrl": String::from("preview_image_thumbnail_url"),
                        "previewImageUrl": String::from("preview_image_url"),
                        "displayUrl": String::from("display_url"),

                    }
                ],
            })
        );
    }
}