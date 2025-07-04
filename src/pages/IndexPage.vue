<script setup lang="ts">
import { onMounted } from "vue";

import AddProject from "@/components/AddProject.vue";
import Project from "@/components/Project.vue";
import { useAppStore } from "@/stores/app";
import { useProjectStore } from "@/stores/project";
import { storeToRefs } from "pinia";

const { getProjects } = storeToRefs(useProjectStore());
onMounted(() => {
	useAppStore().setTitle("BranchWise");
	useProjectStore().setCurrentProject(null);
});
</script>

<template>
	<v-container>
		<v-row no-gutters>
			<v-col
				v-for="project in getProjects"
				:key="project.directory"
				class="mb-4"
				cols="6"
				sm="4"
				md="2"
			>
				<Project :project="project" />
			</v-col>
			<v-col
				class="mb-4"
				cols="6"
				sm="4"
				md="2"
			>
				<AddProject />
			</v-col>
		</v-row>
	</v-container>
</template>
