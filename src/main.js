const { invoke } = window.__TAURI__.core;
const { Menu, Submenu } = window.__TAURI__.menu;
const { platform } = window.__TAURI__.os;

// Tenhou URLs
const tenhouURLs = {
  pairi: "https://tenhou.net/2/",
  game: "https://tenhou.net/3",
  game4k: "https://tenhou.net/4/",
};

const iframe = document.getElementById("main");

function init() {
  // Set up app menu
  setupAppMenu();

  // By default load the 4k game
  iframe.src = tenhouURLs.game4k;
}

async function setupAppMenu() {
  if (platform() === "macos") {
    // const fileSubMenu = await Submenu.new({
    //   items: [
    //     {
    //       id: "quit",
    //       text: "Quit",
    //       action: () => {
    //         console.log("quit pressed");
    //       },
    //     },
    //   ],
    // });
    // const appMenu = await Menu.new({
    //   items: [fileSubMenu],
    // });
    // appMenu.setAsAppMenu().then((res) => {
    //   console.log("menu set success", res);
    // });
  } else {
  }
}

init();
