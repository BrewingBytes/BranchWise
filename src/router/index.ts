import IndexPage from "@/pages/IndexPage.vue";
import ProjectPage from "@/pages/ProjectPage.vue";
import { createMemoryHistory, createRouter } from "vue-router";

const routes = [
	{
		path: "/",
		component: IndexPage
	},
	{
		path: "/project",
		component: ProjectPage
	}
];

export default createRouter({
	history: createMemoryHistory(),
	routes
});
