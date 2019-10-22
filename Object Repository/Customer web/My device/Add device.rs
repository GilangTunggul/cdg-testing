<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Device</name>
   <tag></tag>
   <elementGuidId>8bc843d8-a9a1-4e4d-9326-ffe6c5841914</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;sku&quot;,
      &quot;value&quot;: &quot;test_product&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;name&quot;,
      &quot;value&quot;: &quot;test_product&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;deviceID&quot;,
      &quot;value&quot;: &quot;2&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;suggestedWarranty&quot;,
      &quot;value&quot;: &quot;suggested&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;detail&quot;,
      &quot;value&quot;: &quot;\u003cp\u003etest description product from magento\u003c/p\u003e&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;image&quot;,
      &quot;value&quot;: &quot;http://ec2-18-140-5-8.ap-southeast-1.compute.amazonaws.com/pub/media/catalog/product//p/s/psi.jpg&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;purchaseDate&quot;,
      &quot;value&quot;: &quot;1558540800000&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;remarks&quot;,
      &quot;value&quot;: &quot;da&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;invoice&quot;,
      &quot;value&quot;: &quot;data:image/png;base64,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\u003d\u003d&quot;,
      &quot;type&quot;: &quot;Text&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookies}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}module/cdg/main/customer/product/add</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>81f86c52-1beb-4e71-9814-b7e9822dd404</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookies</defaultValue>
      <description></description>
      <id>a476a4d6-212f-4d94-bb1b-22816ae6a877</id>
      <masked>false</masked>
      <name>cookies</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>e519f2de-79c6-4b55-95f5-eeca463fde12</id>
      <masked>false</masked>
      <name>token</name>
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

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

if(jsonResponse.success == true) {

	
	WS.verifyElementPropertyValue(response, 'message', jsonResponse.message)
	
	WS.verifyElementPropertyValue(response, 'success', true)

} else {

	WS.verifyElementPropertyValue(response, 'message', jsonResponse.message)
	
	WS.verifyElementPropertyValue(response, 'success', false)


} 
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
