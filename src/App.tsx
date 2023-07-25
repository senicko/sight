import { appWindow } from "@tauri-apps/api/window";
import { createEffect, createSignal, onCleanup } from "solid-js";

const fadeOutTime = 200;
const totalBreakTime = 10 * 60 * 1000;

const formatTime = (time: number) => {
  const minutes = Math.floor(time / 60);
  const seconds = time - minutes * 60;

  return `${minutes < 10 ? "0" : ""}${minutes}:${
    seconds < 10 ? "0" : ""
  }${seconds}`;
};

function App() {
  const [time, setTime] = createSignal(totalBreakTime / 1000);
  const [fadingOut, setFadingOut] = createSignal(false);
  let background: HTMLDivElement | undefined;

  setTimeout(() => {
    if (!background) throw new Error("background ref is undefined");

    setFadingOut(true);
  }, totalBreakTime - fadeOutTime);

  createEffect(() => {
    if (fadingOut() && background) {
      background.classList.remove("fade-in");
      background.classList.add("fade-out");

      setTimeout(() => {
        appWindow.close();
      }, fadeOutTime);
    }
  });

  const timer = setInterval(() => {
    setTime(time() - 1);
  }, 1000);

  onCleanup(() => clearInterval(timer));

  const formattedTime = () => formatTime(time());

  return (
    <div ref={background} class="background fade-in">
      <span>take_a_break(When::Now);</span>
      <span class="time">{formattedTime()}</span>
      <button class="postpone_button" onClick={() => setFadingOut(true)}>
        postpone();
      </button>
    </div>
  );
}

export default App;
