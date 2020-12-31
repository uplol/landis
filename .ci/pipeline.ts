import { Workspace, pushStep, spawnChildJob } from "runtime/core.ts";
import * as Docker from "pkg/buildy/docker@latest/mod.ts";
import { readSecrets } from "runtime/secrets.ts";

const image = "rust:1.48-buster";

export async function run(ws: Workspace) {
    const tag = `landis/landis:latest`;

    pushStep(`Build Landis Image`);
    await Docker.buildImage({
        tag: tag,
    });

    pushStep(`Push Landis Image`);
    const [githubUsername, githubToken] = await readSecrets(
        "GITHUB_USERNAME",
        "GITHUB_TOKEN",
    );

    await Docker.pushImage(
        `${tag}`,
        "docker.pkg.github.com/uplol",
        {
            username: githubUsername,
            password: githubToken,
        },
    );
}
