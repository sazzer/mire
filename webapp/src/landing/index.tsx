import { Authentication } from "./authentication";
import React from "react";
import landingPicture from "./landing.jpg";

/**
 * The landing page, containing the available login options.
 */
export const LandingPage: React.FC = () => {
  return (
    <div className="row">
      <aside className="col-12 col-lg-3 order-lg-3">
        <Authentication />
      </aside>
      <div className="col-12 col-lg-9">
        <img
          src={landingPicture}
          alt="Mire"
          className="img-fluid img-thumbnail rounded shadow"
        />
      </div>
    </div>
  );
};
