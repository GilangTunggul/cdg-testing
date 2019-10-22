<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Reschedule</name>
   <tag></tag>
   <elementGuidId>b44e149c-f83f-46ed-85c3-018f137c7964</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;newData\&quot; : {\n\t    \&quot;storeID\&quot;: \&quot;5bfe0cc7fd73d8321cd23cc0\&quot;,\n\t    \&quot;scheduledAt\&quot;: \&quot;1546153211000\&quot;,\n\t    \&quot;description\&quot;: \&quot;consult about product\&quot;,\n\t    \&quot;type\&quot;: \&quot;submit\&quot;,\n\t    \&quot;ticketID\&quot;: \&quot;5ce268253f6dee03153c9133\&quot;\n\t},\n\t\&quot;oldData\&quot; : {\n\t\t\&quot;_id\&quot; : \&quot;5ce612ce3f6dee007e4a6652\&quot;,\n\t\t\&quot;status\&quot; : \&quot;rescheduled\&quot;\n\t},\n\t\&quot;ticket\&quot; : {\n    \t\&quot;_id\&quot; : \&quot;5ce4f0c23f6dee00574aa2a5\&quot;,\n    \t\&quot;scheduleSubmitted\&quot; : \&quot;1546153211000\&quot;\n    }\n}&quot;,
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
   <restUrl>${url}api/cdg/customer/appoinments/reschedules</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>2a370299-96aa-4127-929b-9d7f94bb36aa</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenapp</defaultValue>
      <description></description>
      <id>6bfd5496-eb36-4f55-9134-0163d2ed0206</id>
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

JsonSlurper slurper = new JsonSlurper()

Map parsedJson = slurper.parseText(response.getResponseText())

parsedJson.get('data')

parsedJson.get('message')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
