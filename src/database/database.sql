CREATE TABLE info
(
	id SERIAL PRIMARY KEY,
	info_hash char(40) NOT NULL,
	announce_url varchar(80) NOT NULL,
	creation_date BIGINT NOT NULL,
	title text NOT NULL,
	unique(info_hash,announce_url)
);

CREATE TABLE stats
(
	stats_id SERIAL references info(id),
	poll_time BIGINT NOT NULL,
	downloaded BIGINT NOT NULL,
	seeding BIGINT NOT NULl,
	incomplete BIGINT NOT NULL
);