-- Your SQL goes here

CREATE TABLE base_news (
  id SERIAL PRIMARY KEY,
  tag TEXT NOT NULL,
  title TEXT NOT NULL,
  n_image_urls INTEGER,
  image_urls TEXT[] NOT NULL,
  n_likes INTEGER,
  n_comments INTEGER,
  src_website TEXT,
  craw_time TIMESTAMP NOT NULL ,
  create_time TIMESTAMP NOT NULL
);

ALTER SEQUENCE base_news_id_seq RESTART WITH 1;

INSERT INTO base_news (tag, title, n_image_urls, image_urls, n_likes, n_comments, src_website, craw_time, create_time)
  VALUES ('推荐', '两仪式厨上线', 1,
  '{"https://img.moegirl.org/common/thumb/7/7e/215%E6%BB%A1%E7%A0%B4.png/250px-215%E6%BB%A1%E7%A0%B4.png"}',
  99, 44, 'MOE', '2019-12-16 19:51:25-00', '2018-1-1 12:12:12-00');