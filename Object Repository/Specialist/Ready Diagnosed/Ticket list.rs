<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Ticket list</name>
   <tag></tag>
   <elementGuidId>219662ad-a5cc-4d6d-ab5c-a2f9f5a08bde</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;function&quot;,
      &quot;value&quot;: &quot;list&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$match][status][$in][]&quot;,
      &quot;value&quot;: &quot;diagnosed&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$match][$or][0][productName][$regex]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$match][$or][0][productName][$options]&quot;,
      &quot;value&quot;: &quot;i&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$match][$or][1][ticketNumber][$regex]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$match][$or][1][ticketNumber][$options]&quot;,
      &quot;value&quot;: &quot;i&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$sort][0][field]&quot;,
      &quot;value&quot;: &quot;createdAt&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$sort][0][order]&quot;,
      &quot;value&quot;: &quot;-1&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$skip]&quot;,
      &quot;value&quot;: &quot;0&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$limit]&quot;,
      &quot;value&quot;: &quot;8&quot;,
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
   <restUrl>${url}module/cdg/main/specialist/product/list</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>bd5df914-e371-4c09-bbf2-4964b13f14fd</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookiesspecialist</defaultValue>
      <description></description>
      <id>84108ae9-9d38-4b1a-bb46-452b74eedd44</id>
      <masked>false</masked>
      <name>cookies</name>
   </variables>
   <variables>
      <defaultValue>['burn in', 'completed', 'repair', 'diagnosed']</defaultValue>
      <description></description>
      <id>eed76d85-a13f-45b0-89d9-b6f35cb9455c</id>
      <masked>false</masked>
      <name>status</name>
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

if (jsonResponse.data == []) {

	WS.verifyElementPropertyValue(response, 'success', 'false')
	
	WS.verifyElementPropertyValue(response, 'message', '[Empty ticket]')

} else {

	println(jsonResponse.data != [])
	
	WS.verifyElementPropertyValue(response, 'success', 'true')

	WS.verifyElementPropertyValue(response, 'message', '[]')

}

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
