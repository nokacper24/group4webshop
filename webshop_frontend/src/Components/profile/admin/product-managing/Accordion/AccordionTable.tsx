import { useEffect, useState } from "react";
import { AccordionSection, AccordionSectionProps } from "./AccordionSection";
import { ChangeType } from "./ChangeTypes";
import { showHeaderPopup } from "../Edit-popups/HeaderEditPopup";
import { Description, Testimonial } from "../../../../../Interfaces";
import { TestimonialSection } from "../testimonials/TestimonialSection";

export type AccordionTableProps = {
  sections: AccordionSectionProps[];
  testimonials: Testimonial[];
  productID: string;
  registerContentChange: (id: number, type: ChangeType) => void;
  setTestimonials: (testimonials: Testimonial[]) => void;
  setSections: (sections: AccordionSectionProps[]) => void;
};

/**
 * Renders a table consisting of the header and a body with rows
 * connected to the header through this component.
 *
 * @returns The React component for the Accordion table
 */
export default function AccordionTable(props: AccordionTableProps) {
  /**
   * Deletes a section, including its header and rows in the body, from the table.
   * Makes react re-render.
   *
   * @param id The ID of the section that has changed
   */
  const deleteSection = (id: number) => {
    const section = props.sections.find((section) => section.sectionID === id);
    section?.rows.forEach((row) => {
      props.registerContentChange(row.component_id, ChangeType.Delete);
    });
    const newSections = props.sections.filter(
      (section) => section.sectionID !== id
    );
    props.setSections(newSections);
  };

  /**
   * Creates a new section in the table.
   * Makes react re-render.
   *
   * @param title The title of the new section
   */
  const newSection = () => {
    const id = props.sections.length;
    showHeaderPopup({ title: undefined, informationCallBack: finishCreation });
    function finishCreation(title: string) {
      props.sections.push({
        header: {
          title: title,
        },
        rows: [],
        sectionID: id,
      });
      props.setSections([...props.sections]);
    }
  };


  return (
    <div className="accordion-table">
      <button
        className="default-button small-button popup-button"
        onClick={() => newSection()}
      >
        New section
      </button>
      {props.sections.map((section) => (
        <AccordionSection
          key={section.sectionID}
          header={section.header}
          rows={section.rows}
          sectionID={section.sectionID}
          registerContentChange={props.registerContentChange}
          deleteSection={deleteSection}
        />
      ))}
      <TestimonialSection
        testimonials={props.testimonials}
        sectionId={0}
        productId={props.productID}
        setTestimonials={props.setTestimonials}
      />
    </div>
  );
}
