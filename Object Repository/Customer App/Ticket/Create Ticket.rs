<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Ticket</name>
   <tag></tag>
   <elementGuidId>1bf31cb0-167b-4935-a0f2-3f285f3c05a0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;ticketService\&quot; : {\n\t    \&quot;productNumber\&quot;: \&quot;test_product\&quot;,\n\t    \&quot;productName\&quot;: \&quot;Test_product\&quot;,\n\t    \&quot;sku\&quot;: \&quot;test_product\&quot;,\n\t    \&quot;case\&quot;: {\n\t        \&quot;issues\&quot;: [\n\t            {\n\t                \&quot;id\&quot;: \&quot;5c2c41594d8e205ecc157d82\&quot;,\n\t                \&quot;text\&quot;: \&quot;Lcd\&quot;\n\t            }\n\t        ],\n\t        \&quot;detail\&quot;: \&quot;from mobile\&quot;\n\t    }\n\t},\n\t\&quot;scheduleSubmit\&quot; : {\n\t\t\&quot;storeID\&quot; : \&quot;5c2c41594d8e205ecc157d82\&quot;,\n\t\t\&quot;scheduledAt\&quot; : \&quot;1558079518365\&quot;\n\t}\n}&quot;,
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
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}api/cdg/customer/tickets/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>b6963c80-468a-4a7f-9e3f-3c149b601161</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>b343753b-4390-415c-9113-fb2473be8d51</id>
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

assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 400))

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

if (response.getStatusCode() == 200) {

println(jsonResponse != null)

WS.verifyElementPropertyValue(response, 'message', 'success')

} else {

WS.verifyElementPropertyValue(response, 'message', '[Tickets available]')

}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
