import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import java.sql.Connection as Connection
import java.sql.ResultSet as ResultSet
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
import com.katalon.plugin.keyword.connection.DBType as DBType

// doc: https://github.com/rosiecorn/katalon-dbms-plugin-usage
// https://jsonformatter.org/json-to-jsonschema
globalConnection = CustomKeywords.'com.katalon.plugin.keyword.connection.DatabaseKeywords.getGlobalConnection'()

globalConnection.setReadOnly(false)

actorData = CustomKeywords.'com.katalon.plugin.keyword.connection.DatabaseKeywords.executeQuery'(globalConnection, 'SELECT * FROM shop')

println(CustomKeywords.'com.katalon.plugin.keyword.connection.ResultSetKeywords.getListCellValue'(actorData, 'raw_domain'))

CustomKeywords.'com.katalon.plugin.keyword.connection.DatabaseKeywords.executeUpdate'(globalConnection, 'INSERT INTO logs (testcase_id, status, data) VALUES (\'134\', \'32\', \'huy test2\');')

WS.sendRequest(findTestObject('AR Reviews Widget Hub/Update Reviewbox Setting - param', [('authorization') : 'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJkZXN0IjoiaHR0cHM6XC9cL2h1eW5oLWFyLW5ld3dpZGdldC1zdGFnLWRhaWx5LTEwMDIubXlzaG9waWZ5LmNvbSIsImlhdCI6MTcwMzQ3ODQ4NiwiZXhwIjoxNzAzNDgyMDg2fQ.cVGqQLVdQumf85kR4bDWDTMpoqrxQ_YCPbg3amrlAN4'
            , ('accessory_text_font_size') : 12, ('ask_a_question_mode') : 'new_section', ('auto_play_speed') : 5, ('body_text_font_size') : 14
            , ('border_radius') : 6, ('box_shadow') : 'none', ('button_background') : '', ('button_outline_color') : '', ('button_text_color') : ''
            , ('card_background_color') : '', ('card_border_color') : '', ('columns_on_mobile') : 0, ('custom_css') : ''
            , ('customer_avatar') : 'auto', ('customer_name_format') : 'full_name', ('date_format') : 'll', ('default_questions_sorting') : ''
            , ('default_reviews_sorting') : '', ('enable_auto_play') : '', ('enable_infinite_loop') : '', ('font_family') : 'inherit'
            , ('font_size') : 0, ('heading_text_font_size') : 0, ('hide_for_products_without_reviews') : true, ('layout') : ''
            , ('margin_bottom') : 0, ('margin_top') : 0, ('max_width') : 0, ('number_of_reviews_slider') : 0, ('primary_text_color') : ''
            , ('qna_customer_name_format') : '', ('qna_date_format') : '', ('qna_show_customer_country_flag') : true, ('qna_show_positive_reactions_only') : true
            , ('qna_show_published_date') : true, ('qna_show_reaction_icon') : true, ('rating_icon_border_color') : '', ('rating_icon_empty_color') : ''
            , ('rating_icon_filled_color') : '', ('rating_icon_shape') : '', ('rating_icon_size') : 0, ('rating_icon_spacing') : 0
            , ('reviews_per_loading_grid') : 0, ('reviews_per_loading_list') : 0, ('secondary_text_color') : '', ('section_background') : ''
            , ('show_average_rating') : true, ('show_card_border') : true, ('show_cta_buttons') : true, ('show_custom_questions_rating') : true
            , ('show_filter') : true, ('show_media_section') : true, ('show_positive_reactions_only') : true, ('show_published_date') : true
            , ('show_qna_section') : true, ('show_questions_search_box') : true, ('show_questions_sorting') : true, ('show_rating_distribution') : true
            , ('show_rating_icon_border') : true, ('show_reaction_icon') : true, ('show_review_media') : true, ('show_review_star') : true
            , ('show_reviewer_country_flag') : true, ('show_search_box') : true, ('show_sorting') : true, ('show_summary_section') : true
            , ('show_tabs') : false, ('show_title') : true, ('show_verified_badge') : true, ('slide_behavior') : '', ('verified_badge_color') : ''
            , ('write_a_review_mode') : '', ('your_brand_color') : '']))

