import DemoApp from './DemoApp.svelte';

const debugDiv = document.getElementById("svelte-debug");
if ( debugDiv != null ) {
    const app = new DemoApp({
        target: debugDiv
    });
} else {
    window.DemoApp = DemoApp;
}

