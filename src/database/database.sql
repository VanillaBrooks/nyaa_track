-- CREATE TABLE info (
-- 	id SERIAL PRIMARY KEY,
-- 	info_hash char(20) NOT NULL,
-- 	announce_url varchar(80) NOT NULL,
-- 	creation_date integer NOT NULL,
-- 	title text NOT NULL,
-- 	self_created_info bool NOT NULL
-- );
	
-- CREATE TABLE stats (
-- 	stats_id integer references info(id),
-- 	downloaded integer NOT NULL,
-- 	seeding integer NOT NULl,
-- 	incomplete integer NOT NULL
-- );



CREATE TABLE info
(
	id SERIAL PRIMARY KEY,
	info_hash char(40) NOT NULL UNIQUE,
	announce_url varchar(80) NOT NULL,
	creation_date BIGINT NOT NULL,
	title text NOT NULL
);

CREATE TABLE stats
(
	stats_id SERIAL references info(id),
	downloaded BIGINT NOT NULL,
	seeding BIGINT NOT NULl,
	incomplete BIGINT NOT NULL
);