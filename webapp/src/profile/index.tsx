import { Authenticated } from "../components/authenticated";
import React from "react";

/**
 * Page for viewing and editing the user profile
 */
export const ProfilePage: React.FC = () => {
  return (
    <Authenticated>
      <div>Hello</div>
    </Authenticated>
  );
};
