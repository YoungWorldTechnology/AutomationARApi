<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>shopify metafields</name>
   <tag></tag>
   <elementGuidId>8f5c6b1a-edc1-46b1-89ab-50ae88f9e79b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;{ currentAppInstallation { id launchUrl review_box_status: metafield(namespace: \\\&quot;app_block\\\&quot;, key: \\\&quot;review_box\\\&quot;) { id value type createdAt updatedAt } review_box_setting: metafield(namespace: \\\&quot;app_block\\\&quot;, key: \\\&quot;review_box_setting\\\&quot;) { id value type createdAt updatedAt } review_box_contents: metafield(namespace: \\\&quot;app_block\\\&quot;, key: \\\&quot;review_box_contents\\\&quot;) { id value type createdAt updatedAt } } }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;{\n  currentAppInstallation {\n      id\n      launchUrl\n    review_box_status: metafield(namespace: \&quot;app_block\&quot;, key: \&quot;review_box\&quot;) {\n        id\n      value\n      type\n      createdAt\n      updatedAt\n    }\n    review_box_setting: metafield(namespace: \&quot;app_block\&quot;, key: \&quot;review_box_setting\&quot;) {\n        id\n      value\n      type\n      createdAt\n      updatedAt\n    }\n    review_box_contents: metafield(namespace: \&quot;app_block\&quot;, key: \&quot;review_box_contents\&quot;) {\n        id\n      value\n      type\n      createdAt\n      updatedAt\n    }\n  }\n}\n&quot;,
  &quot;displayVariables&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-Shopify-Access-Token</name>
      <type>Main</type>
      <value>${access_token}</value>
      <webElementGuid>3c377994-d752-4fc6-b428-8f6c770f0693</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>79769479-798c-436e-847c-00041b432e0a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${raw_domain}/admin/api/2023-01/graphql.json</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'huynh-ar-newwidget-stag-daily-1002.myshopify.com'</defaultValue>
      <description></description>
      <id>59d1be86-72f2-4da8-a74c-dc57161c6c52</id>
      <masked>false</masked>
      <name>raw_domain</name>
   </variables>
   <variables>
      <defaultValue>'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'</defaultValue>
      <description></description>
      <id>3ae99e28-253d-4733-8b9c-7c4c8e5e2c61</id>
      <masked>false</masked>
      <name>access_token</name>
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
