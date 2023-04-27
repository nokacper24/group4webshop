export default function TestimonialPopup(
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
