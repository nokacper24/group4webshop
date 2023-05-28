import { RefObject, useEffect, useRef, useState } from "react";
import { Testimonial } from "../../../../../Interfaces";

/**
 * Props for the TestimonialPopup component.
 * @param testimonial The testimonial to be edited. If undefined, a new testimonial will be created.
 * @param informationCallBack A callback function that will be called when the popup is closed. The testimonial that was edited will be passed as a parameter.
 */
type TestimonialPopupProps = {
  testimonial: Testimonial | undefined;
  informationCallBack: (testimonial: Testimonial) => void;
};

type TestimonialPopupInitializationProps = {
  product_id: string;
};

let popupRef: RefObject<HTMLDivElement>;
let authorNameRef: RefObject<HTMLInputElement>;
let authorPicRef: RefObject<HTMLInputElement>;
let testimonialText: RefObject<HTMLTextAreaElement>;
let updatePropsFunc: (newProps: TestimonialPopupProps) => void;

let product_id: string;
export default function TestimonialPopup(
  initializationProps: TestimonialPopupInitializationProps
) {
  popupRef = useRef(null);
  authorNameRef = useRef(null);
  authorPicRef = useRef(null);
  testimonialText = useRef(null);

  product_id = initializationProps.product_id;

  const [props, setProps] = useState<TestimonialPopupProps>({
    testimonial: undefined,
    informationCallBack: () => {},
  });

  /**
   * Stops people from just pressing enter to submit the form.
   *
   * @param event The event that triggered the function.
   */
  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
  }

  const updateProps = (newProps: TestimonialPopupProps) => {
    setProps(newProps);
  };

  /**
   * Saves the testimonial and closes the popup.
   */
  const save = () => {
    //Uses the callback function to return the values of the testimonial.
    props.informationCallBack({
      testimonial_id: 0,
      author: authorNameRef.current?.value!,
      text: testimonialText.current?.value!,
      author_pic: authorPicRef.current?.value!,
      product_id: product_id,
    });
    hidePopup();
  };

  useEffect(() => {
    updatePropsFunc = updateProps;
  });

  authorPicRef.current?.addEventListener("change", () => {
    props.testimonial!.author_pic = authorPicRef.current?.files![0]
      ? authorPicRef.current?.files![0]
      : "";
  });

  return (
    <div className="popup-grey-zone" ref={popupRef}>
      <div>
        <form
          className="form-container container"
          onSubmit={(event) => {
            handleSubmit(event);
          }}
        >
          <h2>Edit testimonial</h2>
          <label htmlFor="authorName">Author name:</label>
          <input
            type="text"
            ref={authorNameRef}
            id="authorName"
            name="authorName"
            defaultValue={props.testimonial ? props.testimonial.author : ""}
          />
          <label htmlFor="authorPic">Author picture:</label>
          <input
            type="file"
            id="authorPic"
            name="authorPic"
            alt="Author picture"
            accept="image/png, image/jpeg, image/webp"
            onChange={() => {
              setProps({
                ...props,
                testimonial: {
                  ...props.testimonial!,
                  author_pic: authorPicRef.current?.value!,
                },
              });
            }}
            ref={authorPicRef}
          />
          <p>
            Current image:{" "}
            {typeof props.testimonial?.author_pic === "string"
              ? props.testimonial?.author_pic
              : props.testimonial?.author_pic.name}
          </p>
          <label htmlFor="testimonial-text">Testimonial:</label>
          <textarea
            name="testimonial-text"
            id="testimonial-text"
            cols={40}
            rows={10}
            ref={testimonialText}
            defaultValue={props.testimonial ? props.testimonial.text : ""}
          ></textarea>

          <div className="button-container popup-button-container">
            <button
              className="default-button small-button popup-button"
              type="button"
              onClick={() => save()}
            >
              Add
            </button>
            <button
              className="default-button small-button popup-button"
              type="button"
              onClick={() => hidePopup()}
            >
              Cancel
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

/**
 * Shows the testimonial popup.
 *
 * @param inProps The props for the testimonial popup.
 */
export function showTestimonialPopup(inProps: TestimonialPopupProps) {
  updatePropsFunc(inProps);
  popupRef.current?.classList.add("popup-visible");
}

/**
 * Hides the testimonial popup by removing the popup-visible class from the popup.
 */
function hidePopup() {
  popupRef.current?.classList.remove("popup-visible");
}
