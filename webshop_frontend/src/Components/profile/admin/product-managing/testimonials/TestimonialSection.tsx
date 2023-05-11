import { useState } from "react";
import { TestimonialHeader } from "./TestimonialHeader";
import { TestimonialBody } from "./TestimonialBody";
import { showTestimonialPopup } from "../Edit-popups/TestimonialPopup";
import { Testimonial } from "../../../../../Interfaces";

/**
 * The props of the TestimonialSection component.
 */
export type TestimonialSectionProps = {
  testimonials: Testimonial[];

  sectionId: number;
  productId: string;
  setTestimonials: (testimonials: Testimonial[]) => void;
};

let latestID = 100;
/**
 * The main component of the Testimonial section. This component is responsible for rendering the header and the body of the section.
 *
 * @param props the props of the component
 * @returns the React component for the Testimonial section
 */
export function TestimonialSection(props: TestimonialSectionProps) {
  /**
   * Initializes the process of creating a new row.
   */
  const newTestimonial = () => {
    showTestimonialPopup({
      testimonial: undefined,
      informationCallBack: finishCreation,
    });
    //Function that is called when the user presses save on the popup.
    function finishCreation(testimonial: Testimonial) {
      testimonial = {
        ...testimonial,
        product_id: props.productId,
        testimonial_id: createID(),
      };
      props.testimonials.push(testimonial);
      props.setTestimonials([...props.testimonials]);
    }
  };

  const createID = (): number => {
    return latestID++; //TODO: May need to take a look at implementing a better way of doing this.
  };

  /**
   * Deletes a testimonial from the table.
   *
   * @param id the ID of the testimonial to be deleted
   */
  const deleteTestimonial = (id: number) => {
    let newTestimonials = props.testimonials.filter(
      (testimonial) => testimonial.testimonial_id !== id
    );
    props.setTestimonials(newTestimonials);
  };

  /**
   * Initializes the process of changing the content of a testimonial.
   *
   * @param id the ID of the testimonial to be edited
   */
  const editTestimonial = (id: number) => {
    let testimonial = props.testimonials.find(
      (testimonial) => testimonial.testimonial_id === id
    );
    if (testimonial) {
      showTestimonialPopup({
        testimonial: testimonial,
        informationCallBack: finishEdit,
      });
      //Function that is called when the user presses save on the popup.
      function finishEdit(testimonial: Testimonial) {
        props.testimonials[props.testimonials.indexOf(testimonial)] =
          testimonial;
        props.setTestimonials([...props.testimonials]);
      }
    }
  };

  const [collapsed, setCollapsed] = useState<boolean>(false);

  const toggleCollapse = () => {
    setCollapsed(!collapsed);
  };

  return (
    <>
      <TestimonialHeader
        addTestimonial={newTestimonial}
        toggleCollapse={toggleCollapse}
      ></TestimonialHeader>
      <TestimonialBody
        testimonials={props.testimonials}
        collapsed={collapsed}
        editTestimonial={editTestimonial}
        deletTestimonial={deleteTestimonial}
      ></TestimonialBody>
    </>
  );
}
