import { Link, Route, Switch } from "react-router-dom";

import { HeaderBar } from "./header";
import React from "react";
import { useTranslation } from "react-i18next";

/**
 * Component representing the entire UI.
 */
export const App = () => {
  const { t } = useTranslation();

  return (
    <>
      <HeaderBar />
      <main className="container-fluid">
        <Link to="/hello">Learn React</Link>
        <Switch>
          <Route path="/hello">
            <p>{t("hello")}</p>
          </Route>
        </Switch>
      </main>
    </>
  );
};
