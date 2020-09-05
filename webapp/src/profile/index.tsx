import { Accordion } from "../components/accordion";
import { Authenticated } from "../components/authenticated";
import { Authentications } from "./authentications";
import { Breadcrumb } from "../components/breadcrumb";
import { ProfileForm } from "./profileForm";
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
              title: t("profile.profile.header"),
              content: <ProfileForm />,
            },
            {
              title: t("profile.authentications.header"),
              content: <Authentications />,
            },
          ]}
        />
      </Authenticated>
    </>
  );
};
