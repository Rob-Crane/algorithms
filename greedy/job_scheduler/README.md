# Job Scheduler

Greedy algorithm to schedule jobs.  Each job has a weight and length property.  Optimization objective is to minimize the weighted sum of completion times.  A job's completion time is its length plus the length of preceding jobs in schedule.

Scheduler prioritizes by either `weight-length` or `weight/length` (in both cases ties are broken by `weight`).  The latter produces an optimal solution.

## Usage

Input file has format:
```
[number_of_jobs]
[job_1_weight] [job_1_length]
[job_2_weight] [job_2_length]
```
Example:
```
python3 job_scheduler.py --difference '/tmp/jobs.txt'
python3 job_scheduler.py --ratio '/tmp/jobs.txt'
```

