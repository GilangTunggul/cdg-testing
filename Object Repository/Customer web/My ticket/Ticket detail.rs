<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Ticket detail</name>
   <tag></tag>
   <elementGuidId>67a9a633-2324-4645-9d51-b62dda3055ff</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
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
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookies}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}module/cdg/main/customer/ticket/get/${id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>7e8833bb-84a4-4af9-b893-66214d8f321f</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>'5d96ac5dd194554f732db1e2'</defaultValue>
      <description></description>
      <id>6fc2d376-ffc4-463f-af82-eefea68e2157</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookies</defaultValue>
      <description></description>
      <id>c870f574-e466-44ef-a047-493bc4466c34</id>
      <masked>false</masked>
      <name>cookies</name>
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

println(jsonResponse.data)

def variables = request.getVariables()
def id = variables.get('id')

if (jsonResponse.data._id.$oid == id &amp;&amp; jsonResponse.data.serviceProduct.status == 'new'){
	
	WS.verifyElementPropertyValue(response, 'data._id.$oid', jsonResponse.data._id.$oid)
	
	WS.verifyElementPropertyValue(response, 'data.cases', jsonResponse.data.cases)
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.status', 'new')
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.productNumber', jsonResponse.data.serviceProduct.productNumber)
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.specialistID', jsonResponse.data.serviceProduct.specialistID)
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.verificationDetail', '')
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.repair', null)
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.pending', null)
	
} else {

	WS.verifyElementPropertyValue(response, 'data._id.$oid', jsonResponse.data._id.$oid)
	
	WS.verifyElementPropertyValue(response, 'data.cases', jsonResponse.data.cases)
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.productNumber', jsonResponse.data.serviceProduct.productNumber)
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.specialistID', jsonResponse.data.serviceProduct.specialistID)
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.verificationDetail', '[]')
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.repair', '[]')
	
	WS.verifyElementPropertyValue(response, 'data.serviceProduct.pending', '[]')
	

}






</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
