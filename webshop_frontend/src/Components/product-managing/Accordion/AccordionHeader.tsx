import { AccordionRow, AccordionRowProps } from "./AccordionRow";

export type AccordionHeaderProps = {
  rows: AccordionRowProps[];
};

export function AccordionHeader(props: AccordionHeaderProps) {
  

  return (
    <div>
      <button className="accordion">Testing</button>
      <button className="accordion-up-down-button">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
          <title>Caret Down</title>
          <path d="M98 190.06l139.78 163.12a24 24 0 0036.44 0L414 190.06c13.34-15.57 2.28-39.62-18.22-39.62h-279.6c-20.5 0-31.56 24.05-18.18 39.62z" />
        </svg>
      </button>
      <div className="accordion-body">
        {props.rows?.map((row) => {
          return (
            <AccordionRow
              key={"ACR" + row.id}
              id={row.id}
              title={row.title}
              editFunction={row.editFunction}
              removeFunction={row.removeFunction}
            ></AccordionRow>
          );
        })}
      </div>
    </div>
  );
}
