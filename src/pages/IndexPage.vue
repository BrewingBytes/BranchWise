<template>
    <v-container>
        <v-btn @click="openGitFolder">
            Open Git Project
        </v-btn>

        <v-container v-if="gitProject.state === 'valid'">
            <p>Local branches in {{ gitProject.directory }}</p>
            <p v-for="branch in gitProject.localBranches" :key="branch">
                {{ branch }}
            </p>
            <p>Upstream branches:</p>
            <p v-for="branch in gitProject.remoteBranches" :key="branch">
                {{ branch }}
            </p>
            <p>Tags:</p>
            <p v-for="tag in gitProject.tags" :key="tag">
                {{ tag }}
            </p>
        </v-container>
    </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { IGitProject, DEFAULT_GIT_PROJECT } from "../types/gitProject";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
    name: "IndexPage",
    data() {
        const gitProject: IGitProject = DEFAULT_GIT_PROJECT;

        return {
            gitProject
        };
    },
    methods: {
        async openGitFolder() {
            const result = await open({
                directory: true,
                multiple: false
            });

            if (result) {
                try {
                    this.gitProject = await invoke("open_git_project", { directory: result });

                    console.log(this.gitProject);
                } catch (error) {
                    console.error(error);
                }
            }
        }
    }
});
</script>
