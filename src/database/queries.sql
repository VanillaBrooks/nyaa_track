-- $1 - name of error string
-- $2 - info hash string
-- $3 poll time

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
group by error.info_id, info.info_hash;


--- fetch most recent updates for torrents by time stamp
select info.title, (select to_timestamp(max(stats.poll_time))) from info
inner join stats on stats.stats_id = info.id

where info.title like '%Horrible%' and info.title like '%Kimetsu%' and info.title like '%720p%'
group by info.id;


-- average number of requests over the last 5 minutes
select count(*)/5.0 from stats where ( (SELECT extract(epoch from now())) - poll_time <= (60*5));
