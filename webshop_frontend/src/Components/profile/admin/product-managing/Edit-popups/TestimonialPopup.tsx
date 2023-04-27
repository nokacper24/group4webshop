export default function TestimonialPopup(
  return (
    <div className="popup-grey-zone" ref={popupRef}>
      <div className="popup-box">
        <form
          className="popup-form"
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
              props = {
                ...props,
                testimonial: {
                  ...props.testimonial!,
                  author_pic: authorPicRef.current?.value!,
                },
              };
            }}
            ref={authorPicRef}
          />
          <p>
            Current image:{" "}
            {props.testimonial?.author_pic
              ? props.testimonial?.author_pic
              : "none"}
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

          <div>
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

export function showTestimonialPopup(inProps: TestimonialPopupProps) {
  props = inProps;
  popupRef.current?.classList.add("popup-visible");
}

function save() {
  props.informationCallBack({
    testimonial_id: 0,
    author: authorNameRef.current?.value!,
    text: testimonialText.current?.value!,
    author_pic: authorPicRef.current?.value!,
    product_id: product_id,
  });
  hidePopup();
}

function hidePopup() {
  popupRef.current?.classList.remove("popup-visible");
}
