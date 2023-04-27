import { useEffect, useState } from "react";
import { AccordionSection, AccordionSectionProps } from "./AccordionSection";
import { ChangeType } from "./ChangeTypes";
import { showHeaderPopup } from "../Edit-popups/HeaderEditPopup";
import { Testimonial } from "../../../../../Interfaces";
import { TestimonialSection } from "../testimonials/TestimonialSection";

export type AccordionTableProps = {
  sections: AccordionSectionProps[];
  testimonials: Testimonial[];
  productID: string;
};

/**
 * Renders a table consisting of the header and a body with rows connected to the header through
 * this component.
 *
 * @returns The React component for the Accordion table
 */
export default function AccordionTable(props: AccordionTableProps) {
  const [priorityChanges] = useState<Map<number, number>>(
    new Map()
  ); //ID as key, priority as value
  const [contentChanges, setContentChanges] = useState<
    Map<ChangeType, number[]>
  >(new Map()); //The type of change as key, list of IDs that has had that change as value

  useEffect(() => {
    // Sets up the map so that it can register changes
    setContentChanges((changes) => {
      for (let type in ChangeType) {
        changes.set(ChangeType[type as keyof typeof ChangeType], []);
      }
      return changes;
    });
  });

  /**
   * Registers changes to the content of the table. This is used to keep track of what has changed and what needs to be saved.
   *
   * @param id The ID of the section that has changed
   * @param change The type of change that has been made
   */
  const registerContentChange = (id: number, change: ChangeType) => {
    if (!contentChanges.get(change)?.includes(id)) {
      contentChanges.get(change)?.push(id);
    }
    if (change === ChangeType.Delete) {
      contentChanges
        .get(ChangeType.Edit)
        ?.filter((changeId) => changeId !== id);
      priorityChanges.delete(id);
    }
  };

  /**
   * Deletes a section, including its header and rows in the body, from the table.
   * Makes react re-render.
   *
   * @param id The ID of the section that has changed
   */
  const deleteSection = (id: number) => {
    const index = sectionList.findIndex((section) => section.sectionID === id);

    const newSections = sectionList.filter(
      (section) => section.sectionID !== id
    );
    console.log(newSections);
    setSectionList(newSections);
  };

  const [sectionList, setSectionList] = useState<AccordionSectionProps[]>([
    ...props.sections,
  ]);

  /**
   * Creates a new section in the table.
   * Makes react re-render.
   *
   * @param title The title of the new section
   */
  const newSection = () => {
    const id = sectionList.length;
    showHeaderPopup({ title: undefined, informationCallBack: finishCreation });
    function finishCreation(title: string) {
      sectionList.push({
        header: {
          title: title,
        },
        rows: [],
        sectionID: id,
        registerContentChange: registerContentChange,
        deleteSection: deleteSection,
      });
      setSectionList([...sectionList]);
    }
    console.log(sectionList);
  };

  return (
    <div className="accordion-table">
      <button
        className="default-button small-button popup-button"
        onClick={() => newSection()}
      >
        New section
      </button>
      {sectionList.map((section) => 
        (
          <AccordionSection
            key={section.sectionID}
            header={section.header}
            rows={section.rows}
            sectionID={section.sectionID}
            registerContentChange={registerContentChange}
            deleteSection={deleteSection}
          />
        )
      )}
      <TestimonialSection
        testimonials={props.testimonials}
        sectionId={0}
        productId={props.productID}
      />
    </div>
  );
}
