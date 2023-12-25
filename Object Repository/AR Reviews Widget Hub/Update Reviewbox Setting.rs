<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Reviewbox Setting</name>
   <tag></tag>
   <elementGuidId>0d4a7562-2c0a-40aa-9078-fc815dba02f1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;${payload}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>3c377994-d752-4fc6-b428-8f6c770f0693</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>authorization</name>
      <type>Main</type>
      <value>${authorization}</value>
      <webElementGuid>e2038697-f7b4-4286-b98e-bdb935c97467</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.base_url_new_widget}/api/widgets/review_box</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJkZXN0IjoiaHR0cHM6XC9cL2h1eW5oLWFyLW5ld3dpZGdldC1zdGFnLWRhaWx5LTEwMDIubXlzaG9waWZ5LmNvbSIsImlhdCI6MTcwMzQ3NDU3MCwiZXhwIjoxNzAzNDc4MTcwfQ.Bvq-225KToM9TdhMR_rgJbVmsUII4qog3pXD6zqPEYM'</defaultValue>
      <description></description>
      <id>59d1be86-72f2-4da8-a74c-dc57161c6c52</id>
      <masked>false</masked>
      <name>authorization</name>
   </variables>
   <variables>
      <defaultValue>'{\r\n  &quot;setting&quot;: {\r\n    &quot;accessory_text_font_size&quot;: 12,\r\n    &quot;ask_a_question_mode&quot;: &quot;new_section&quot;,\r\n    &quot;auto_play_speed&quot;: 5,\r\n    &quot;body_text_font_size&quot;: 14,\r\n    &quot;border_radius&quot;: 6,\r\n    &quot;box_shadow&quot;: &quot;none&quot;,\r\n    &quot;button_background&quot;: &quot;rgba(40,41,61,1)&quot;,\r\n    &quot;button_outline_color&quot;: &quot;rgba(228,228,235,1)&quot;,\r\n    &quot;button_text_color&quot;: &quot;rgba(255,255,255,1)&quot;,\r\n    &quot;card_background_color&quot;: &quot;rgba(255,255,255,1)&quot;,\r\n    &quot;card_border_color&quot;: &quot;rgba(228,228,235,1)&quot;,\r\n    &quot;columns_on_mobile&quot;: 1,\r\n    &quot;custom_css&quot;: null,\r\n    &quot;customer_avatar&quot;: &quot;auto&quot;,\r\n    &quot;customer_name_format&quot;: &quot;full_name&quot;,\r\n    &quot;date_format&quot;: &quot;ll&quot;,\r\n    &quot;default_questions_sorting&quot;: &quot;newest_first&quot;,\r\n    &quot;default_reviews_sorting&quot;: &quot;by_date&quot;,\r\n    &quot;enable_auto_play&quot;: true,\r\n    &quot;enable_infinite_loop&quot;: true,\r\n    &quot;font_family&quot;: &quot;inherit&quot;,\r\n    &quot;font_size&quot;: 14,\r\n    &quot;heading_text_font_size&quot;: 26,\r\n    &quot;hide_for_products_without_reviews&quot;: false,\r\n    &quot;layout&quot;: &quot;grid&quot;,\r\n    &quot;links_color&quot;: &quot;rgba(40,41,61,1)&quot;,\r\n    &quot;margin_bottom&quot;: 20,\r\n    &quot;margin_top&quot;: 20,\r\n    &quot;max_width&quot;: 1200,\r\n    &quot;number_of_reviews_slider&quot;: 8,\r\n    &quot;primary_text_color&quot;: &quot;rgba(40,41,61,1)&quot;,\r\n    &quot;qna_customer_name_format&quot;: &quot;full_name&quot;,\r\n    &quot;qna_date_format&quot;: &quot;ll&quot;,\r\n    &quot;qna_show_customer_country_flag&quot;: true,\r\n    &quot;qna_show_positive_reactions_only&quot;: false,\r\n    &quot;qna_show_published_date&quot;: true,\r\n    &quot;qna_show_reaction_icon&quot;: true,\r\n    &quot;rating_icon_border_color&quot;: &quot;rgba(255,122,0,1)&quot;,\r\n    &quot;rating_icon_empty_color&quot;: &quot;rgba(204,204,204,1)&quot;,\r\n    &quot;rating_icon_filled_color&quot;: &quot;rgba(250,173,20,1)&quot;,\r\n    &quot;rating_icon_shape&quot;: &quot;default&quot;,\r\n    &quot;rating_icon_size&quot;: 14,\r\n    &quot;rating_icon_spacing&quot;: 2,\r\n    &quot;reviews_per_loading_grid&quot;: 12,\r\n    &quot;reviews_per_loading_list&quot;: 5,\r\n    &quot;secondary_text_color&quot;: &quot;rgba(143,144,166,1)&quot;,\r\n    &quot;section_background&quot;: &quot;rgba(255,255,255,1)&quot;,\r\n    &quot;show_average_rating&quot;: true,\r\n    &quot;show_card_border&quot;: true,\r\n    &quot;show_cta_buttons&quot;: true,\r\n    &quot;show_custom_questions_rating&quot;: true,\r\n    &quot;show_filter&quot;: true,\r\n    &quot;show_media_section&quot;: false,\r\n    &quot;show_positive_reactions_only&quot;: false,\r\n    &quot;show_published_date&quot;: true,\r\n    &quot;show_qna_section&quot;: true,\r\n    &quot;show_questions_search_box&quot;: true,\r\n    &quot;show_questions_sorting&quot;: true,\r\n    &quot;show_rating_distribution&quot;: true,\r\n    &quot;show_rating_icon_border&quot;: false,\r\n    &quot;show_reaction_icon&quot;: true,\r\n    &quot;show_review_media&quot;: true,\r\n    &quot;show_review_star&quot;: true,\r\n    &quot;show_reviewer_country_flag&quot;: true,\r\n    &quot;show_search_box&quot;: true,\r\n    &quot;show_sorting&quot;: true,\r\n    &quot;show_summary_section&quot;: true,\r\n    &quot;show_tabs&quot;: true,\r\n    &quot;show_title&quot;: true,\r\n    &quot;show_verified_badge&quot;: true,\r\n    &quot;slide_behavior&quot;: &quot;one_item&quot;,\r\n    &quot;verified_badge_color&quot;: &quot;rgba(6,136,80,1)&quot;,\r\n    &quot;write_a_review_mode&quot;: &quot;new_section&quot;,\r\n    &quot;your_brand_color&quot;: &quot;rgba(33,33,33,1)&quot;\r\n  },\r\n  &quot;contents&quot;: [\r\n    {\r\n      &quot;language_code&quot;: &quot;en&quot;,\r\n      &quot;content&quot;: {\r\n        &quot;summary_section_title&quot;: &quot;Customer Reviews 19&quot;,\r\n        &quot;summary_review_singular_format&quot;: &quot;[review_number] review 1&quot;,\r\n        &quot;summary_review_plural_format&quot;: &quot;[review_number] reviews&quot;,\r\n        &quot;summary_review_submit_label&quot;: &quot;Write A Review&quot;,\r\n        &quot;summary_qna_submit_label&quot;: &quot;Ask A Question&quot;,\r\n        &quot;summary_discount_amount&quot;: &quot;Get discount [discount_amount]&quot;,\r\n        &quot;review_empty_title&quot;: &quot;There are no customer reviews yet&quot;,\r\n        &quot;review_empty_content&quot;: &quot;Be the first to share your thoughts with other customers.&quot;,\r\n        &quot;qna_empty_title&quot;: &quot;There are no questions yet&quot;,\r\n        &quot;qna_empty_content&quot;: &quot;Don’t hesitate to share your concerns with us.&quot;,\r\n        &quot;review_load_more&quot;: &quot;Load more&quot;,\r\n        &quot;review_count&quot;: &quot;Reviews ([reviews_count])&quot;,\r\n        &quot;qna_question_count&quot;: &quot;Questions ([questions_count])&quot;,\r\n        &quot;review_filter_by&quot;: &quot;Filter&quot;,\r\n        &quot;review_filter_clear&quot;: &quot;Clear&quot;,\r\n        &quot;review_sort_by_date&quot;: &quot;Sort by date&quot;,\r\n        &quot;review_sort_by_rating&quot;: &quot;Sort by rating&quot;,\r\n        &quot;review_sort_by_content&quot;: &quot;Sort by content&quot;,\r\n        &quot;qna_sort_by_date&quot;: &quot;Sort by date&quot;,\r\n        &quot;review_section_title&quot;: &quot;Write A Review&quot;,\r\n        &quot;review_rating_label&quot;: &quot;Rating&quot;,\r\n        &quot;review_rating_empty&quot;: &quot;Please give us your rating&quot;,\r\n        &quot;review_upload_label&quot;: &quot;Add Media&quot;,\r\n        &quot;review_upload_max_photo_size&quot;: &quot;Maximum upload photo size: 10MB&quot;,\r\n        &quot;review_upload_max_video_size&quot;: &quot;Maximum upload video size: 100MB&quot;,\r\n        &quot;review_upload_max_attachments&quot;: &quot;Maximum attachments: 5 items&quot;,\r\n        &quot;review_upload_incorrect_format&quot;: &quot;The file must be .jpg, .png, .gif, .mp4 or .mov&quot;,\r\n        &quot;review_content_label&quot;: &quot;Feedback&quot;,\r\n        &quot;review_content_placeholder&quot;: &quot;Share your thought about the product&quot;,\r\n        &quot;review_content_required&quot;: &quot;This field is required&quot;,\r\n        &quot;review_name_label&quot;: &quot;Name&quot;,\r\n        &quot;review_name_placeholder&quot;: &quot;Enter your name&quot;,\r\n        &quot;review_name_required&quot;: &quot;This field is required&quot;,\r\n        &quot;review_email_label&quot;: &quot;Email&quot;,\r\n        &quot;review_email_placeholder&quot;: &quot;Enter your mail&quot;,\r\n        &quot;review_email_invalid&quot;: &quot;Invalid email format&quot;,\r\n        &quot;review_email_required&quot;: &quot;This field is required&quot;,\r\n        &quot;review_submit_label&quot;: &quot;Submit Review&quot;,\r\n        &quot;review_cancel_label&quot;: &quot;Cancel&quot;,\r\n        &quot;review_discount_suggestion&quot;: &quot;Upload review with photos or videos to get [discount_amount] discount instantly!&quot;,\r\n        &quot;review_thank_you_title&quot;: &quot;Thank you!&quot;,\r\n        &quot;review_thank_you_content&quot;: &quot;Thank you for being one of our success stories!&quot;,\r\n        &quot;review_discount_content&quot;: &quot;Take your Discount [discount_amount] coupon now&quot;,\r\n        &quot;review_shop_now&quot;: &quot;Shop now&quot;,\r\n        &quot;review_discount_expiry_date&quot;: &quot;Expiry date: [expiry_date]&quot;,\r\n        &quot;qna_section_title&quot;: &quot;Write A Question&quot;,\r\n        &quot;qna_question_label&quot;: &quot;Question&quot;,\r\n        &quot;qna_question_placeholder&quot;: &quot;Write your question here&quot;,\r\n        &quot;qna_question_required&quot;: &quot;This field is required&quot;,\r\n        &quot;qna_name_label&quot;: &quot;Name&quot;,\r\n        &quot;qna_name_placeholder&quot;: &quot;Enter your name&quot;,\r\n        &quot;qna_name_required&quot;: &quot;This field is required&quot;,\r\n        &quot;qna_email_label&quot;: &quot;Email&quot;,\r\n        &quot;qna_email_placeholder&quot;: &quot;Enter your mail&quot;,\r\n        &quot;qna_email_invalid&quot;: &quot;Invalid email format&quot;,\r\n        &quot;qna_email_required&quot;: &quot;This field is required&quot;,\r\n        &quot;qna_submit_label&quot;: &quot;Submit Question&quot;,\r\n        &quot;qna_cancel_label&quot;: &quot;Cancel&quot;,\r\n        &quot;qna_thank_you_title&quot;: &quot;Thank you!&quot;,\r\n        &quot;qna_thank_you_content&quot;: &quot;Thank you for being one of our success stories!&quot;,\r\n        &quot;review_sort_by_media&quot;: &quot;Sort by media&quot;,\r\n        &quot;qna_sort_by_date_newest&quot;: &quot;Newest first&quot;,\r\n        &quot;qna_sort_by_date_oldest&quot;: &quot;Oldest first&quot;,\r\n        &quot;qna_load_more&quot;: &quot;Load more&quot;,\r\n        &quot;qna_search_box_placeholder&quot;: &quot;Search for questions&quot;,\r\n        &quot;qna_see_more&quot;: &quot;See more&quot;,\r\n        &quot;qna_helpful&quot;: &quot;Was this helpful?&quot;,\r\n        &quot;review_search_box_placeholder&quot;: &quot;Search for reviews&quot;,\r\n        &quot;review_see_more&quot;: &quot;See more&quot;,\r\n        &quot;review_verified_purchase&quot;: &quot;Verified purchase&quot;,\r\n        &quot;review_not_verified_purchase&quot;: &quot;Rating&quot;,\r\n        &quot;review_helpful&quot;: &quot;Was this helpful?&quot;,\r\n        &quot;review_empty&quot;: &quot;There are no customer reviews yet.&quot;,\r\n        &quot;custom_question_required&quot;: &quot;This field is required&quot;,\r\n        &quot;media_gallery_title&quot;: &quot;All media&quot;\r\n      }\r\n    }\r\n  ]\r\n}'</defaultValue>
      <description></description>
      <id>3ae99e28-253d-4733-8b9c-7c4c8e5e2c61</id>
      <masked>false</masked>
      <name>payload</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
