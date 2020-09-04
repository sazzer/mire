import { Route, Switch } from "react-router-dom";

import { HeaderBar } from "./header";
import { LandingPage } from "./landing";
import { ProfilePage } from "./profile";
import React from "react";

/**
 * Component representing the entire UI.
 */
export const App: React.FC = () => {
  return (
    <>
      <HeaderBar />
      <main className="container-fluid">
        <Switch>
          <Route path="/profile">
            <ProfilePage />
          </Route>
          <Route path="/">
            <LandingPage />
          </Route>
        </Switch>
      </main>
    </>
  );
};
