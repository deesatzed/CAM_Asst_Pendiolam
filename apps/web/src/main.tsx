import { StrictMode, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import { LandingPage } from "./LandingPage";
import "./styles.css";

function Root() {
  const [studio, setStudio] = useState(() => window.location.hash === "#studio");

  useEffect(() => {
    const onHashChange = () => setStudio(window.location.hash === "#studio");
    window.addEventListener("hashchange", onHashChange);
    return () => window.removeEventListener("hashchange", onHashChange);
  }, []);

  return studio ? <App /> : <LandingPage />;
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Root />
  </StrictMode>,
);
