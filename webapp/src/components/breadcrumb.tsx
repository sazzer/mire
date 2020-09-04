import { Link } from "react-router-dom";
import React from "react";

/**
 * Shape of the details for a single crumb
 */
export interface Crumb {
  /** The link target for the crumb */
  link: string;
  /** The text for the crumb */
  text: string;
}

/**
 * Props for the Breadcrumbs
 */
export interface BreadcrumbProps {
  /** The crumbs leading up to the current page */
  crumbs?: Crumb[];
  /** The label for the current page */
  current: string;
}

/**
 * Component to render breadcrumbs
 */
export const Breadcrumb: React.FC<BreadcrumbProps> = ({ crumbs, current }) => {
  const crumbLinks = crumbs?.map((crumb) => {
    return (
      <li className="breadcrumb-item" key={crumb.link}>
        <Link to={crumb.link}>{crumb.text}</Link>
      </li>
    );
  });

  return (
    <nav aria-label={current}>
      <ol className="breadcrumb">
        {crumbLinks}
        <li className="breadcrumb-item active" aria-current="page">
          {current}
        </li>
      </ol>
    </nav>
  );
};
