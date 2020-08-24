import { Link, Route, Switch } from "react-router-dom";

import React from "react";
import { useTranslation } from "react-i18next";

export const App = () => {
  const { t } = useTranslation();

  return (
    <div className="App">
      <header className="App-header">
        <Link to="/hello">Learn React</Link>
        <Switch>
          <Route path="/hello">
            <p>{t("hello")}</p>
          </Route>
        </Switch>
      </header>
    </div>
  );
};
