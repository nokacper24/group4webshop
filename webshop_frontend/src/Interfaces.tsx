export interface License {
  license_id: number;
  valid: boolean;
  start_date: Date;
  end_date: Date;
  amount: number;
  company_id: number;
  product_id: string;
  product_name: string;
}

export interface LicenseVital {
  license_id: number;
  company_id: number;
  company_name: string;
  product_id: string;
  display_name: string;
  valid: boolean;
}

export interface User {
  user_id: string;
  email: string;
  pass_hash: string;
  company_id: number;
  role: string;
}

export interface PartialUser {
  email?: string;
}

export interface MeUser {
  user_id: number;
  email: string;
  role: string;
  company_id: number;
}

export interface Company {
  company_id: number;
  company_name: string;
  company_address: string;
}

export interface Product {
  product_id: string;
  display_name: string;
  price_per_user: number;
  short_description: string;
  main_image: string;
  available: boolean;
}

export interface Text {
  text_title: string;
  paragraph: string;
}

export interface Image {
  image_path: string;
  alt_text: string;
}

export interface Description {
  component_id: number;
  priority: number;
  product_id: string;
  text: Text;
  image: Image;
  is_text_not_image: boolean;
  full_width: boolean;
}

export interface Testimonial {
  testimonial_id: number;
  author: string;
  text: string;
  author_pic: string;
  product_id: string;
}
