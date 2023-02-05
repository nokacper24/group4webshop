import { SlideType } from "./SlideTypes";


export type slideProps = {
  mainContent: string;
  reviewerProfile: {
    picturePath: string;
    name: string;
    title: string;
  };
  slideType: SlideType;
};

export default function () {
  return <div></div>;
}
