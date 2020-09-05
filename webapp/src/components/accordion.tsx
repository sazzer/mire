import React, { useState } from "react";

export interface Pane {
  title: string;
  content: React.ReactNode;
}

export interface AccordionProps {
  id: string;
  panes: Pane[];
}

export const Accordion: React.FC<AccordionProps> = ({ id, panes }) => {
  const [active, setActive] = useState(0);

  const cards = panes.map((pane, index) => {
    const isActive = active === index;
    const cardId = `${id}-card-${index}`;

    const buttonClasses = [
      "btn",
      "btn-link",
      "btn-block",
      "text-left",
      isActive ? "" : "collapsed",
    ].join(" ");
    const paneClass = isActive ? "collapse show" : "collapse";

    return (
      <div className="card" key={index}>
        <div className="card-header" id={cardId + "-heading"}>
          <h2 className="mb-0">
            <button
              className={buttonClasses}
              type="button"
              aria-expanded={isActive}
              aria-controls={cardId}
              onClick={() => setActive(index)}
            >
              {pane.title}
            </button>
          </h2>
        </div>

        <div
          id={cardId}
          className={paneClass}
          aria-labelledby={cardId + "-heading"}
        >
          <div className="card-body">{isActive && pane.content}</div>
        </div>
      </div>
    );
  });
  return (
    <div className="accordion" id={id}>
      {cards}
    </div>
  );
};
