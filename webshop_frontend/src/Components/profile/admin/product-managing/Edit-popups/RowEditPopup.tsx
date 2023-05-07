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
  content: string | undefined;
  informationCallBack: (image: boolean, title: string, content: string) => void;
};

let popupRef: RefObject<HTMLDivElement>;
let imageRef: RefObject<HTMLInputElement>;
let titleRef: RefObject<HTMLInputElement>;
let altTextRef: RefObject<HTMLTextAreaElement>;
let paragraphRef: RefObject<HTMLTextAreaElement>;
let updatePropsFunc: (newProps: RowEditPopupProps) => void;
export default function RowEditPopup() {
  popupRef = useRef(null);
  imageRef = useRef(null);
  titleRef = useRef(null);
  altTextRef = useRef(null);
  paragraphRef = useRef(null);

  let [props, setProps] = useState<RowEditPopupProps>({
    image: false,
    title: undefined,
    content: undefined,
    informationCallBack: () => {},
  });

  const updateProps = (newProps: RowEditPopupProps) => {
    setProps(newProps); //TODO: Remove nested ternary operators
    if (titleRef.current) {
      titleRef.current.value = newProps.title ? newProps.title : "";
    }
    if (paragraphRef.current) {
      paragraphRef.current.value = newProps.content ? newProps.content : "";
    }
    if (altTextRef.current) {
      altTextRef.current.value = newProps.title ? newProps.title : "";
    }
  };

  const save = () => {
    let content: string;
    if (props.image) {
      content = imageRef.current ? imageRef.current.value : "";
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

  function changeImageState() {
    setProps({ ...props, image: !props.image });
  }

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
  }

  useEffect(() => {
    updatePropsFunc = updateProps;
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
                  setProps({ ...props, content: imageRef.current?.value })
                }
                ref={imageRef}
                defaultValue={props.content ? props.content : ""}
              />
              <p>Current image: {props.content}</p>
              <label htmlFor="alt-text">Alt-text:</label>
              <textarea
                name="alt-text"
                id="alt-text"
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
                defaultValue={props.content ? props.content : ""}
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
