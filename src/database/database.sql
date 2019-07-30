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



--- fetch most recent updates for torrents by time stamp
select info.title, (select to_timestamp(max(stats.poll_time))) from info
inner join stats on stats.stats_id = info.id

where info.title like '%Horrible%' and info.title like '%Kimetsu%'
group by info.id;

-- $1 - name of error string
-- $2 - info hash string
-- $3 poll time
-- 

with
	type_id_
	as
	(
		select type_id
		from error_types
		where error_name = $1
	),
	info_id_
	as
	(
		select id
		from info
		where info_hash = $2
	)
insert into error
	(err_type, info_id, poll_time)
VALUES
	(
		(select *from type_id_), 
		(select *from info_id_), 
		$3);

		

with type_id_ as ( select type_id from error_types where error_name = $1 ), info_id_ as ( select id from info where info_hash = $2 ) insert into error (err_type, info_id, poll_time) VALUES ( (select * from type_id_), (select * from info_id_), $3);



-- total the number of errors for each hash (and get the last update of that hash)
select info.info_hash, error.info_id, count(error.info_id), (select to_timestamp(max(poll_time)) from stats where stats_id=error.info_id)
from error
inner join info on info.id = error.info_id
group by error.info_id, info.info_hash



-- get the total number of requests sent over the last 100 seconds
create view benchmark as 
	with unix_time as (select extract(epoch from now()))
	select count(stats_id) from stats where poll_time >= ((select * from unxi_time) - 100) and poll_time <= (select * from unix_time);


-- fetch the distinc ids that have been updated in the last hour
create view current_track as 
	with unix_time as (select extract(epoch from now()))
	select count(DISTINCT stats_id) from stats where poll_time >= ((select *  from unix_time) - 3600) and poll_time <= (select * from unix_time) 