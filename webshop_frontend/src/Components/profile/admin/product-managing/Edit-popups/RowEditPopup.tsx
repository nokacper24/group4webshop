import { RefObject, useEffect, useRef, useState } from "react";

/**
 * Props for the RowEditPopup component.
 * Image indicates if the content should be interpreted as an image or not.
 * The title and content can be undefined if a new row is being created.
 * If the content is an image, the title is the alt text.
 */
export type RowEditPopupProps = {
  image: boolean;
  title: string | undefined;
  content: string | File | undefined;
  informationCallBack: (
    image: boolean,
    title: string,
    content: string | File
  ) => void;
};

let popupRef: RefObject<HTMLDivElement>;
let imageInputRef: RefObject<HTMLInputElement>;
let titleRef: RefObject<HTMLInputElement>;
let altTextRef: RefObject<HTMLTextAreaElement>;
let paragraphRef: RefObject<HTMLTextAreaElement>;
let updatePropsFunc: (newProps: RowEditPopupProps) => void;

/**
 * The popup for editing a row.
 *
 * @returns The RowEditPopup component
 */
export default function RowEditPopup() {
  popupRef = useRef(null);
  imageInputRef = useRef(null);
  titleRef = useRef(null);
  altTextRef = useRef(null);
  paragraphRef = useRef(null);

  let [props, setProps] = useState<RowEditPopupProps>({
    image: false,
    title: undefined,
    content: undefined,
    informationCallBack: () => {},
  });

  /**
   * Updates the props of the component with the parameter.
   *
   * @param newProps The new props of the component.
   */
  const updateProps = (newProps: RowEditPopupProps) => {
    setProps(newProps); //TODO: Remove nested ternary operators
    if (titleRef.current) {
      titleRef.current.value = newProps.title ? newProps.title : "";
    }
    if (paragraphRef.current && typeof newProps.content === "string") {
      paragraphRef.current.value = newProps.content ? newProps.content : "";
    }
    if (altTextRef.current) {
      altTextRef.current.value = newProps.title ? newProps.title : "";
    }
  };

  /**
   * Saves the row and closes the popup. If the form is invalid, the user will be alerted and the popup will not close.
   *
   * @returns void
   */
  const save = () => {
    if (!validateForm()) return;
    let content: string;
    if (props.image) {
      content = imageInputRef.current ? imageInputRef.current.value : "";
    } else {
      content = paragraphRef.current?.value ? paragraphRef.current.value : "";
    }
    let title: string;
    if (props.image) {
      title = altTextRef.current?.value ? altTextRef.current.value : "";
    } else {
      title = titleRef.current?.value ? titleRef.current.value : "";
    }
    props.informationCallBack(props.image, title, content);
    hidePopup();
  };

  let changeStateButtonText = props.image
    ? "Change to paragraph"
    : "Change to image";

  /**
   * Changes the state of the popup between image and paragraph.
   */
  function changeImageState() {
    setProps({ ...props, image: !props.image });
  }

  /**
   * Default handleSubmit function to prevent the form from incorrectly submitting.
   *
   * @param event The event that triggered the function.
   */
  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
  }

  useEffect(() => {
    updatePropsFunc = updateProps;
  });

  imageInputRef.current?.addEventListener("change", () => {
    props.content = imageInputRef.current?.files?.[0];
  });

  /**
   * Validates the form and alerts the user if there are any fields not filled or
   * filled incorrectly.
   *
   * @returns true if the form is valid, false otherwise.
   */
  function validateForm(): boolean {
    if (!props.image) {
      if (
        paragraphRef.current?.value === undefined ||
        RegExp(/^ *$/).exec(paragraphRef.current?.value) !== null
      ) {
        alert("Paragraph cannot be empty");
        return false;
      }
      if (paragraphRef.current?.value.length > 255) {
        alert("Paragraph cannot be longer than 255 characters");
        return false;
      }
      if (
        titleRef.current?.value === undefined ||
        RegExp(/^ *$/).exec(titleRef.current?.value) !== null ||
        titleRef.current?.value.length > 255
      ) {
        alert("Title cannot be empty or longer than 255 characters");
        return false;
      }
    } else {
      if (imageInputRef.current?.files?.[0] === undefined) {
        alert("Please upload an image");
        return false;
      }
      if (altTextRef.current?.value === undefined ||
        RegExp(/^ *$/).exec(altTextRef.current?.value) !== null ||
        altTextRef.current?.value.length > 255) {
        alert("Alt-text cannot be empty or longer than 255 characters");
        return false;
      }
    }
    
    return true;
  }

  return (
    <div className="popup-grey-zone" ref={popupRef}>
      <div>
        <form
          className="form-container container"
          method="POST"
          onSubmit={(event) => {
            handleSubmit(event);
          }}
        >
          {props.image ? (
            <>
              <h2>Edit image</h2>
              <label htmlFor="image">Image:</label>
              <input
                type="file"
                id="image"
                name="image"
                accept="image/png, image/jpeg, image/webp"
                onChange={() =>
                  setProps({ ...props, content: imageInputRef.current?.value })
                }
                ref={imageInputRef}
                defaultValue={
                  props.content && typeof props.content === "string"
                    ? props.content
                    : ""
                }
              />
              <p>Current image: </p>
              <label htmlFor="alt_text">Alt-text:</label>
              <textarea
                name="alt_text"
                id="alt_text"
                cols={40}
                rows={10}
                ref={altTextRef}
                defaultValue={props.title ? props.title : ""}
              ></textarea>
            </>
          ) : (
            <>
              <h2>Edit paragraph</h2>
              <label htmlFor="title">Title:</label>
              <input
                type="text"
                id="title"
                name="title"
                defaultValue={props.title ? props.title : ""}
                ref={titleRef}
              />
              <label htmlFor="paragraph">Paragraph:</label>
              <textarea
                id="paragraph"
                defaultValue={
                  props.content && typeof props.content === "string"
                    ? props.content
                    : ""
                }
                cols={40}
                rows={10}
                ref={paragraphRef}
              />
            </>
          )}
          <button
            className="default-button small-button popup-button"
            type="button"
            onClick={() => changeImageState()}
          >
            {changeStateButtonText}
          </button>
          <div className="button-container popup-button-container">
            <button
              className="default-button small-button popup-button"
              type="button"
              onClick={() => save()}
            >
              Save
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

export function showPopup(inProps: RowEditPopupProps) {
  updatePropsFunc(inProps);
  popupRef.current?.classList.add("popup-visible");
}

function hidePopup() {
  popupRef.current?.classList.remove("popup-visible");
}
