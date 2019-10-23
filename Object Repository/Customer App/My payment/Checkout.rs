<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Checkout</name>
   <tag></tag>
   <elementGuidId>69e244ef-6f85-4c5b-b627-d3bf41c8d366</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;serviceID\&quot;: \&quot;5d1c09073f6dee012b6d7183\&quot;,\n    \&quot;paymentID\&quot;: \&quot;5d1c09083f6dee012b6d7186\&quot;,\n    \&quot;customerID\&quot;: \&quot;5c347ef218ab712b173386e9\&quot;,\n    \&quot;total\&quot;: 80,\n    \&quot;currency\&quot;: \&quot;SGD\&quot;,\n    \&quot;paymentMethod\&quot;: \&quot;paypal\&quot;,\n    \&quot;description\&quot;: \&quot;service fee\&quot;,\n    \&quot;items\&quot;: [\n        {\n            \&quot;name\&quot;: \&quot;service fee\&quot;,\n            \&quot;qty\&quot;: 1,\n            \&quot;price\&quot;: 80,\n            \&quot;subtotal\&quot;: 80,\n            \&quot;confirm\&quot;: true,\n            \&quot;id\&quot;: \&quot;\&quot;\n        }\n    ]\n}&quot;,
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
   <restUrl>${url}api/cdg/paypal/checkout</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>b57694ca-96a3-4d98-87c4-5e8e6420dc48</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenapp</defaultValue>
      <description></description>
      <id>7b8b3ebf-032a-45fb-a924-6bdcd84cc4f8</id>
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

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
