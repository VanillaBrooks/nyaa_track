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

CREATE TABLE error_types
(
	error_name char(20) NOT NULL,
	type_id SERIAL PRIMARY KEY
);

CREATE TABLE error
(
	error_id SERIAL PRIMARY KEY,
	info_id SERIAL REFERENCES info(id),
	err_type INT NOT NULL REFERENCES error_types(type_id),
	poll_time BIGINT NOT NULL
);


INSERT INTO error_types  (error_name) VALUES ('invalid scrape');
INSERT INTO error_types  (error_name) VALUES ('invalid announce');


-- FETCH ids from stats where the announce either is within the last 7 days 
--or has more than 100 seeds at the last scrape
create view ids_to_track
as

	with
		max_polls
		as
		(
			select max(poll_time) as mpt, stats_id as s_id
			from stats
			group by stats_id
		)
	select stats_id
	from stats
		inner join max_polls on stats.stats_id = max_polls.s_id AND max_polls.mpt = stats.poll_time
	where stats.seeding > 100
	order by stats.stats_id;


-- pull relevant data to construct an announce component from the view ids_to_track
create view data_to_track
as
	select info_hash, creation_date, title, announce_url
	from info
	WHERE info.id in (select *
	from ids_to_track) OR (((select extract(epoch from now()) ) - info.creation_date)/86400) <7;



-- get the total number of requests sent over the last 100 seconds
create view benchmark as 
	with unix_time as (select extract(epoch from now()))
	select count(stats_id) from stats where poll_time >= ((select * from unxi_time) - 100) and poll_time <= (select * from unix_time);


-- fetch the distinc ids that have been updated in the last hour
create view current_track as 
	with unix_time as (select extract(epoch from now()))
	select count(DISTINCT stats_id) from stats where poll_time >= ((select *  from unix_time) - 3600) and poll_time <= (select * from unix_time) 