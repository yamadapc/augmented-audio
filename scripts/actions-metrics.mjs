import Bluebird from "bluebird";
import range from "lodash/range.js";
import groupBy from "lodash/groupBy.js";

let cache = {};
try {
  cache = JSON.parse(fs.readFileSync("./jobs-cache.json"));
} catch (err) {}

async function getWorkflowRuns(page = 1) {
  const { workflow_runs } = JSON.parse(
    await $`hub api "/repos/yamadapc/augmented-audio/actions/runs?branch=master&per_page=100&page=${page}"`
  );
  return workflow_runs;
}

const workflowRuns = [].concat(
  ...(await Bluebird.map(range(5), (page) => getWorkflowRuns(page + 1), {
    concurrency: 3,
  }))
);

console.log("");

const allJobs = [].concat(
  ...(await Bluebird.map(
    workflowRuns,
    async (run) => {
      const runJobs =
        cache[run.id] ??
        JSON.parse(
          await $`hub api "/repos/yamadapc/augmented-audio/actions/runs/${run.id}/jobs"`
        );
      cache[run.id] = runJobs;
      const { jobs: rawJobs } = runJobs;

      const jobs = rawJobs
        .filter(
          (rawJob) =>
            rawJob.status === "completed" && rawJob.conclusion === "success"
        )
        .map((rawJob) => {
          return {
            id: rawJob.id,
            name: rawJob.name,
            status: rawJob.status,
            conclusion: rawJob.conclusion,
            startedAt: rawJob.started_at,
            timeToRunMs:
              new Date(rawJob.completed_at) -
              new Date(rawJob.started_at).getTime(),
          };
        });
      // console.log({
      //   name: run.name,
      //   status: run.status,
      //   conclusion: run.conclusion,
      //   jobs: jobs,
      // });
      return jobs;
    },
    {
      concurrency: 3,
    }
  ))
);

fs.writeFileSync("./jobs-cache.json", JSON.stringify(cache, null, 2));

const groupedJobs = groupBy(allJobs, "name");
Object.entries(groupedJobs).forEach(([name, jobs]) => {
  const averageTime =
    jobs.map((job) => job.timeToRunMs).reduce((m, i) => m + i) / jobs.length;
  console.log(JSON.stringify({ name, averageTime, jobsCount: jobs.length }));
});

fs.rmSync("./jobs.csv", { force: true });
fs.appendFileSync(
  "./jobs.csv",
  ["id", "name", "timeToRunMs", "startedAt"].join(",") + "\n"
);
for (let job of allJobs) {
  fs.appendFileSync(
    "./jobs.csv",
    [job.id, job.name, job.timeToRunMs, job.startedAt].join(",") + "\n"
  );
}
