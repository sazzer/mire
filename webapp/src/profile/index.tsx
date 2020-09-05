import { Accordion } from "../components/accordion";
import { Authenticated } from "../components/authenticated";
import { Breadcrumb } from "../components/breadcrumb";
import React from "react";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * Page for viewing and editing the user profile
 */
export const ProfilePage: React.FC = () => {
  const { t } = useTranslation();
  const { user } = useUser();

  return (
    <>
      <Breadcrumb
        current={t("profile.breadcrumb", { displayName: user?.displayName })}
        crumbs={[{ link: "/", text: t("common.home") }]}
      />

      <Authenticated>
        <Accordion
          id="userProfile"
          panes={[
            {
              title: "Profile",
              content: <div>Profile</div>,
            },
            {
              title: "Login details",
              content: <div>Login details</div>,
            },
          ]}
        />
      </Authenticated>
    </>
  );
};
